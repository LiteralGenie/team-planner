from .interpolate_ability_utils import Term

NUM_STAR_LEVELS = 3

STAT_LEVEL_MULTS = dict(
    ad=1.5,
    health=1.8,
)

AP_TERM = lambda var: Term(var, "ap", div=100)
AP_DAMAGE_TERM = AP_TERM("Damage")
PERCENT_ATTACK_TERM = Term("PercentAttackDamage", "ad")

FORMULAS = {
    "TFT11_Aatrox": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedHealing": [Term("FlatHealing"), Term(None, "ap")],
    },
    "TFT11_Ahri": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedSecondaryDamage": [AP_TERM("SecondaryDamage")],
    },
    "TFT11_Alune": {
        "ModifiedDamage": [AP_TERM("MagicDamage")],
        "ModifiedPrimaryDamage": [AP_TERM("PrimaryMagicDamage")],
    },
    "TFT11_Amumu": {
        "ModifiedAllyHealingAmount": [AP_TERM("AllyHealingAmount")],
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedSelfHealingAmount": [AP_TERM("SelfHealingAmount")],
    },
    "TFT11_Annie": {
        "BonusHealth": [Term("HealthRatio", "hp")],
        "PassiveHeal": [AP_TERM("APHealth"), Term("HealRatio", "hp")],
        "TotalDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Aphelios": {
        "ModifiedDamage": [Term("BaseDamage"), PERCENT_ATTACK_TERM],
    },
    "TFT11_Ashe": {
        "ModifiedDamage": [PERCENT_ATTACK_TERM],
    },
    "TFT11_Azir": {
        "ModifiedLaserDamage": [AP_TERM("LaserDamage")],
    },
    "TFT11_Bard": {
        "ModifiedMagicDamage": [AP_TERM("APDamage")],
        "ModifiedPhysicalDamage": [PERCENT_ATTACK_TERM],
        "ScaledDuration": [Term("Duration")],
    },
    "TFT11_Caitlyn": {
        "TotalDamage": [PERCENT_ATTACK_TERM, AP_TERM("APDamage")],
    },
    "TFT11_ChoGath": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedShield": [AP_TERM("Shield")],
    },
    "TFT11_Darius": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedDamageOnHit": [AP_TERM("DamageOnHit")],
    },
    "TFT11_Diana": {
        "ModifiedHeal": [AP_TERM("Heal")],
        "TotalDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Galio": {
        "ModifiedArmor": [Term("BaseResists"), Term("BonusResistsPct", "armor")],
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedMagicResist": [Term("BaseResists"), Term("BonusResistsPct", "resist")],
        "ModifiedShieldAmount": [AP_TERM("ShieldPercent")],
    },
    "TFT11_Garen": {
        "AdditionalDamage": [Term("ADPercent", "ad")],
        "ModifiedShield": [Term("ShieldBase"), Term("PercentHealthShield", "hp")],
    },
    "TFT11_Gnar": {
        "ModifiedADGain": [Term("ADGain", "ad")],
        "TotalDamage": [PERCENT_ATTACK_TERM],
    },
    "TFT11_Hwei": {
        "ModifiedDamage": [AP_TERM("Damage")],
        "ModifiedEndHeal": [AP_TERM("EndHeal")],
        "ModifiedHealing": [AP_TERM("Healing")],
    },
    "TFT11_Illaoi": {
        "ModifiedDamage": [AP_TERM("Damage")],
        "ModifiedHealAmount": [AP_TERM("Heal")],
        "ModifiedShield": [AP_TERM("Shield")],
    },
    "TFT11_Irelia": {
        "Damage_BladeHit": [Term("BladeHitAD", "ad")],
        "Damage_SpellHit": [AP_TERM("SpellAPDamage"), Term("SpellHitAD", "ad")],
    },
    "TFT11_Janna": {
        "ModifiedDamage": [AP_TERM("BaseDamage")],
        "ModifiedShield": [AP_TERM("Shield")],
    },
    "TFT11_Jax": {
        "ModifiedAOEDamage": [AP_TERM("AOEDamage")],
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_KaiSa": {
        "AmplifiedDamage": [Term("AmpedPercentAD", "ad")],
        "ModifiedDamage": [PERCENT_ATTACK_TERM],
    },
    "TFT11_Kayn": {
        "TotalSwipeDamage": [Term("SwipeADPercent", "ad")],
        "TotalTransformDamage": [Term("TransformADPercent", "ad")],
    },
    "TFT11_KhaZix": {
        "TotalDamage": [PERCENT_ATTACK_TERM, AP_TERM("APDamage")],
    },
    "TFT11_Kindred": {
        "ModifiedDamage": [AP_TERM("Damage")],
        "ModifiedSecondaryDamage": [AP_TERM("SecondaryDamage")],
    },
    "TFT11_FortuneYord": {
        "ModifiedDamageAmount": [Term("DamageHPPercent", "hp")],
        "ModifiedHeal": [AP_TERM("Heal")],
    },
    "TFT11_KogMaw": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_LeeSin": {
        "ModifiedAOEDamage": [Term("AOEDamageADRatio", "ad")],
        "ModifiedDamage": [Term("PhysDamageADRatio", "ad")],
        "ModifiedShieldAmount": [AP_TERM("Shield")],
    },
    "TFT11_Lillia": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedMegaDamage": [AP_TERM("MegaDamage")],
        "SecondaryDamage": [AP_TERM("SecondaryTargetDamage")],
    },
    "TFT11_Lissandra": {
        "ModifiedAOEDamage": [AP_TERM("AOEDamage")],
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Lux": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Malphite": {
        "ModifiedArmor": [Term("Armor")],
        "TotalDamage": [Term("PercentArmorDamage", "armor")],
    },
    "TFT11_Morgana": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Nautilus": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Neeko": {
        "ModifiedHeal": [AP_TERM("Heal")],
        "TotalDamage": [AP_TERM("Damage")],
    },
    "TFT11_Ornn": {
        "ModifiedMagicDamage": [AP_TERM("MagicDamage")],
        "ModifiedTotalShieldValue": [
            AP_TERM("BaseShieldValue"),
            Term("PercentHealthShield", "hp"),
        ],
    },
    "TFT11_Qiyana": {
        "ModifiedDamage": [AP_DAMAGE_TERM, PERCENT_ATTACK_TERM],
        "SecondaryDamage": [Term("SecondaryTargetPercentage", "ad")],
    },
    "TFT11_Rakan": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedShield": [AP_TERM("BaseShieldAmount")],
    },
    "TFT11_RekSai": {
        "ModifiedDamage": [AP_DAMAGE_TERM, Term("PercentHPDamage", "hp")],
    },
    "TFT11_Riven": {
        "BoostedModifiedDamage": [Term("PercentAttackDamageBoosted", "ad")],
        "ModifiedDamage": [PERCENT_ATTACK_TERM],
        "ModifiedHeal": [AP_TERM("Heal")],
    },
    "TFT11_Senna": {
        "ModifiedDamage": [PERCENT_ATTACK_TERM, AP_TERM("BaseDamageAPScaling")],
    },
    "TFT11_Sett": {
        "TotalAoEDamage": [
            Term("PercentADSecondary", "ad"),
            AP_TERM("PercentAPSecondary"),
        ],
        "TotalPrimaryDamage": [Term("PercentADPrimary", "ad")],
    },
    "TFT11_Shen": {
        "ModifiedFlatDamageReduction": [AP_TERM("FlatDamageReduction")],
        "ModifiedShenSelfishReduction": [AP_TERM("ShenSelfishReduction")],
        "ModifiedTrueDamage": [Term("PercentArmorDamage", "armor")],
    },
    "TFT11_Sivir": {
        "ModifiedAttackSpeed": [Term("AttackSpeed", "ap")],
    },
    "TFT11_Soraka": {
        "ModifiedAoEDamage": [AP_TERM("AoEDamage")],
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Sylas": {
        "ModifiedAOEDamage": [AP_TERM("AOEDamage")],
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedHeal": [AP_TERM("Heal")],
    },
    "TFT11_Syndra": {
        "ModifiedDamagePerButt": [AP_TERM("DamagePerButt")],
    },
    "TFT11_TahmKench": {
        "BonusModifiedDamage": [Term("BonusPercentDamage", "ap")],
        "BonusModifiedShield": [AP_TERM("BonusShield")],
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedShield": [Term("PercentHealthShield", "hp")],
    },
    "TFT11_Teemo": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Thresh": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedShield": [AP_TERM("Shield")],
    },
    "TFT11_Tristana": {
        "ModifiedDamage": [AP_TERM("BaseSpellDamage"), PERCENT_ATTACK_TERM],
    },
    "TFT11_Udyr": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedShield": [AP_TERM("ShieldAmt")],
        "ModifiedShurikenDamage": [AP_TERM("ShurikenDamage")],
    },
    "TFT11_Volibear": {
        "ModifiedHeal": [AP_TERM("HealAmount")],
        "ModifiedLargeDamage": [AP_TERM("LargeDamage"), Term("HealthRatio", "hp")],
        "ModifiedSmallDamage": [AP_TERM("SmallDamage")],
    },
    "TFT11_Wukong": {
        "ModifiedBonkDamage": [
            AP_TERM("BonkDamage"),
            Term("BonkPercentAttackDamage", "ad"),
        ],
        "ModifiedSpinDamage": [Term("SpinPercentAttackDamage", "ad")],
        "ModifiedStunDamage": [Term("StunPercentAttackDamage", "ad")],
    },
    "TFT11_Xayah": {
        "ModifiedFeatherDamage": [Term("FeatherDamage", "ad")],
        "TotalFeatherRecallDamage": [
            AP_TERM("FeatherRecallBaseDamage"),
            Term("FeatherRecallDamage", "ad"),
        ],
    },
    "TFT11_Yasuo": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
        "ModifiedShield": [
            Term("BaseShield"),
            Term("ShieldResistPct", "armor"),
            Term("ShieldResistPct", "resist"),
        ],
        "ModifiedShieldDamage": [
            Term("ShieldDamage", "armor"),
            Term("ShieldDamage", "resist"),
        ],
    },
    "TFT11_Yone": {
        "TotalDamage": [Term("ADRatio", "ad")],
        "ShieldAmount": [AP_TERM("Shield")],
    },
    "TFT11_Yorick": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Zoe": {
        "ModifiedSecondarySpellDamage": [AP_TERM("SecondarySpellDamage")],
        "TotalDamage": [AP_DAMAGE_TERM],
    },
    "TFT11_Zyra": {
        "ModifiedDamage": [AP_DAMAGE_TERM],
    },
}
FORMULAS = {k.lower(): v for k, v in FORMULAS.items()}
