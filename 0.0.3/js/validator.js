// docs/js/validator.js

// Canonical schema URLs (production)
const SCHEMA_URLS = {
  entity:
    "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.entity.schema.json",
  relationship:
    "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.relationship.schema.json",
  exchange:
    "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.exchange.schema.json",
};

// Optional: local paths for offline / localhost testing
const LOCAL_SCHEMA_PATHS = {
  entity: "../../../schemas/cep.entity.schema.json",
  relationship: "../../../schemas/cep.relationship.schema.json",
  exchange: "../../../schemas/cep.exchange.schema.json",
};

const isLocal =
  !location.hostname || // file:// or weird cases
  location.hostname === "localhost" ||
  location.hostname === "127.0.0.1";

// Helper to pick the right URL based on environment
function getSchemaUrl(schemaKey) {
  if (isLocal) {
    return LOCAL_SCHEMA_PATHS[schemaKey] || LOCAL_SCHEMA_PATHS.entity;
  }
  return SCHEMA_URLS[schemaKey] || SCHEMA_URLS.entity;
}

document$.subscribe(function () {
  const schemaSelect = document.getElementById("schema-select");
  const inputField = document.getElementById("data-to-validate");
  const resultDiv = document.getElementById("validation-result");

  if (!schemaSelect || !inputField || !resultDiv) {
    return;
  }

  // Current AJV validate function (changes when schema changes)
  let currentValidate = null;
  let currentSchemaKey = schemaSelect.value || "entity";

  function setStatus(message, color) {
    resultDiv.textContent = message;
    resultDiv.style.color = color;
  }

  function getAjvCtor() {
    // ajv2020.bundle.js exposes window.ajv2020
    return window.ajv2020 || window.Ajv || null;
  }

  function loadSchemaAndCompile(schemaKey) {
    const url = getSchemaUrl(schemaKey);
    currentSchemaKey = schemaKey;
    setStatus("Loading schema " + schemaKey + "…", "gray");

    const AjvCtor = getAjvCtor();
    if (!AjvCtor) {
      setStatus(
        "Error: Ajv library not loaded (check extra_javascript).",
        "red"
      );
      return;
    }

    setStatus("Loading schema " + schemaKey + "…", "gray");

    fetch(url)
      .then(function (response) {
        if (!response.ok) {
          throw new Error(
            "Failed to fetch schema: " +
              response.status +
              " " +
              response.statusText
          );
        }
        return response.json();
      })
      .then(function (schema) {
        const ajv = new AjvCtor({ allErrors: true, strict: false });

        // Attach standard formats if ajv-formats is loaded
        if (window.ajvFormats) {
          window.ajvFormats(ajv);
        }

        currentValidate = ajv.compile(schema);
        setStatus(
          "Schema loaded. Paste JSON to validate (" + schemaKey + ").",
          "black"
        );

        if (inputField.value.trim().length > 0) {
          validateCurrentInput();
        }
      })
      .catch(function (err) {
        currentValidate = null;
        setStatus("Error loading schema: " + err.message, "red");
      });
  }

  function validateCurrentInput() {
    if (!currentValidate) {
      setStatus("Schema not loaded yet. Please wait…", "gray");
      return;
    }

    let data;
    try {
      data = JSON.parse(inputField.value);
    } catch (e) {
      setStatus("Error: Invalid JSON format: " + e.message, "red");
      return;
    }

    const valid = currentValidate(data);

    if (valid) {
      setStatus(
        "SUCCESS: Record is VALID for schema '" + currentSchemaKey + "'.",
        "green"
      );
    } else {
      const AjvCtor = getAjvCtor();
      let errorsText;

      if (AjvCtor && typeof AjvCtor.errorsText === "function") {
        errorsText = AjvCtor.errorsText(currentValidate.errors || []);
      } else {
        // Fallback: stringify the errors
        errorsText = JSON.stringify(currentValidate.errors || [], null, 2);
      }

      setStatus("FAIL (" + currentSchemaKey + "): " + errorsText, "darkorange");
    }
  }

  // When schema selection changes, reload the schema and recompile
  schemaSelect.addEventListener("change", function () {
    const schemaKey = schemaSelect.value || "entity";
    loadSchemaAndCompile(schemaKey);
  });

  // Validate on input changes
  inputField.addEventListener("input", function () {
    validateCurrentInput();
  });

  // Initial load (default schema: whatever is selected)
  loadSchemaAndCompile(currentSchemaKey);
});
