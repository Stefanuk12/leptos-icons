use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.5 8.5 9 12l-3.5 3.5L2 12l3.5-3.5Z" ></ path > < path d = "m12 2 3.5 3.5L12 9 8.5 5.5 12 2Z" ></ path > < path d = "M18.5 8.5 22 12l-3.5 3.5L15 12l3.5-3.5Z" ></ path > < path d = "m12 15 3.5 3.5L12 22l-3.5-3.5L12 15Z" ></ path > < / > } } pub const LUCIDE_COMPONENT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;