use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m20 17-5-5 5-5" ></ path > < path d = "m4 17 5-5-5-5" ></ path > < / > } } pub const LUCIDE_CHEVRONS_RIGHT_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;