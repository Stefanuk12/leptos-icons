use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M128 64c0-17.7 14.3-32 32-32H320c17.7 0 32 14.3 32 32V416h96V256c0-17.7 14.3-32 32-32H608c17.7 0 32 14.3 32 32s-14.3 32-32 32H512V448c0 17.7-14.3 32-32 32H320c-17.7 0-32-14.3-32-32V96H192V256c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32h96V64z" ></ path > < / > } } pub const FONT_AWESOME_SOLID_WAVE_SQUARE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 640 512")] } ;