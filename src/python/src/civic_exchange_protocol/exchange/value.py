"""Value types for CEP exchanges.

Supports monetary values (with currency) and in-kind contributions.
"""

from dataclasses import dataclass

from civic_exchange_protocol.core import (
    Canonicalize,
    format_amount,
    insert_if_present,
    insert_required,
)


@dataclass
class ValueType:
    """The type of value being exchanged."""

    type_uri: str

    @classmethod
    def monetary(cls) -> "ValueType":
        """Return a ValueType for monetary exchanges."""
        return cls(
            "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/value-type.json#monetary"
        )

    @classmethod
    def in_kind(cls) -> "ValueType":
        """Return a ValueType for in-kind exchanges."""
        return cls(
            "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/value-type.json#in-kind"
        )

    @classmethod
    def service_hours(cls) -> "ValueType":
        """Return a ValueType for service hours exchanges."""
        return cls(
            "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/value-type.json#service-hours"
        )


DEFAULT_VALUE_TYPE_URI = "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/value-type.json#monetary"


@dataclass
class ExchangeValue(Canonicalize):
    """The value being exchanged."""

    amount: float
    currency_code: str = "USD"
    value_type_uri: str = DEFAULT_VALUE_TYPE_URI
    in_kind_description: str | None = None

    @classmethod
    def monetary(cls, amount: float, currency_code: str = "USD") -> "ExchangeValue":
        """Create a new monetary value."""
        return cls(amount=amount, currency_code=currency_code)

    @classmethod
    def usd(cls, amount: float) -> "ExchangeValue":
        """Create a new USD monetary value."""
        return cls.monetary(amount, "USD")

    @classmethod
    def in_kind(cls, amount: float, description: str) -> "ExchangeValue":
        """Create an in-kind value with description."""
        return cls(
            amount=amount,
            currency_code="USD",
            value_type_uri=ValueType.in_kind().type_uri,
            in_kind_description=description,
        )

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical representation of this value as a dictionary.

        Returns:
        -------
        dict[str, str]
            A dictionary containing the canonical fields for this exchange value.
        """
        fields: dict[str, str] = {}
        # Amount formatted to exactly 2 decimal places
        insert_required(fields, "amount", format_amount(self.amount))
        insert_required(fields, "currencyCode", self.currency_code)
        insert_if_present(fields, "inKindDescription", self.in_kind_description)
        insert_required(fields, "valueTypeUri", self.value_type_uri)
        return fields


@dataclass
class ExchangeParty(Canonicalize):
    """A party in an exchange (source or recipient)."""

    entity_id: str
    role_uri: str | None = None
    account_identifier: str | None = None

    def with_role(self, role_uri: str) -> "ExchangeParty":
        """Return a new ExchangeParty with role set."""
        return ExchangeParty(
            entity_id=self.entity_id,
            role_uri=role_uri,
            account_identifier=self.account_identifier,
        )

    def with_account(self, account: str) -> "ExchangeParty":
        """Return a new ExchangeParty with account identifier set."""
        return ExchangeParty(
            entity_id=self.entity_id,
            role_uri=self.role_uri,
            account_identifier=account,
        )

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical representation of this party as a dictionary.

        Returns:
        -------
        dict[str, str]
            A dictionary containing the canonical fields for this exchange party.
        """
        fields: dict[str, str] = {}
        insert_if_present(fields, "accountIdentifier", self.account_identifier)
        insert_required(fields, "entityId", self.entity_id)
        insert_if_present(fields, "roleUri", self.role_uri)
        return fields
