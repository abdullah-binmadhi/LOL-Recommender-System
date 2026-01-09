class FeatureExtractor {
    static calculateBurst(attributes) {
        if (!attributes) return 0;
        const damage = attributes.damage || 0;
        const mobility = attributes.mobility || 0;
        // Weighted formula: 70% Damage, 30% Mobility
        const raw = (damage * 0.7) + (mobility * 0.3);
        // Normalize to 0-100 (Max raw is 3.0)
        return (raw / 3.0) * 100;
    }

    static calculateSustain(attributes) {
        if (!attributes) return 0;
        const toughness = attributes.toughness || 0;
        const utility = attributes.utility || 0;
        // Weighted formula: 70% Toughness, 30% Utility
        const raw = (toughness * 0.7) + (utility * 0.3);
        // Normalize to 0-100 (Max raw is 3.0)
        return (raw / 3.0) * 100;
    }

    static calculateCC(attributes) {
        if (!attributes) return 0;
        const control = attributes.control || 0;
        // Normalize to 0-100 (Max raw is 3.0)
        return (control / 3.0) * 100;
    }
}

if (typeof module !== 'undefined' && module.exports) {
    module.exports = FeatureExtractor;
}