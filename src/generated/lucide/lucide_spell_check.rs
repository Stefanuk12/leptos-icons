use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m6 16 6-12 6 12" ></ path > < path d = "M8 12h8" ></ path > < path d = "m16 20 2 2 4-4" ></ path > < / > } } pub const LUCIDE_SPELL_CHECK : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;