use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m11 17-5-5 5-5" ></ path > < path d = "m18 17-5-5 5-5" ></ path > < / > } } pub const LUCIDE_CHEVRONS_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24")] } ;