SMODS.Atlas {
    key = "modicon",
    path = "icon.png",
    px = 32,
    py = 32
}

if Malverk then
    AltTexture {
        key = 'seal',
        set = 'Seal',
        path = 'malverk/seals.png',
        loc_txt = {
            name = 'Seals',
        }
    }

    AltTexture {
        key = 'cert',
        set = 'Joker',
        keys = { "j_certificate" },
        path = 'malverk/certificate.png',
        loc_txt = {
            name = 'Certificate',
        }
    }


    TexturePack {
        key = 'sealseal', -- the key of the texture
        textures = {'sealseal_seal', 'sealseal_cert'}, -- a table of keys of your AltTexture objects
        loc_txt = { -- Localization text for tooltips displayed in the texture selection screen - can be added to a localization file under [descriptions][texture_packs]
            name = 'SealSeal',
            text = {'Makes seals into seals!'}
        }
    }
else
    print("SealSeal is running without Malverk! Malverk is the recommended way to use this mod, but it will continue in Compatibility mode.")
    SMODS.Atlas {
        key = "centers",
        path = "smods/enhancers.png",
        raw_key = true,

        px = 71,
        py = 95
    }


	SMODS.Atlas({
		key = "Joker",
		px = 71,
		py = 95,
		path = "smods/jokers.png",
		raw_key = true,
	})
end