use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m20 17-5-5 5-5" ></ path > < path d = "m4 17 5-5-5-5" ></ path > < / > } } pub const LUCIDE_CHEVRONS_RIGHT_LEFT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;