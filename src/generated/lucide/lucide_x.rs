use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 6 6 18" ></ path > < path d = "m6 6 12 12" ></ path > < / > } } pub const LUCIDE_X : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;