use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 20 5-5 5 5" ></ path > < path d = "m7 4 5 5 5-5" ></ path > < / > } } pub const LUCIDE_CHEVRONS_DOWN_UP : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;