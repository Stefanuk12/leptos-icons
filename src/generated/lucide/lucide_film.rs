use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" height = "18" y = "3" rx = "2" ></ rect > < path d = "M7 3v18" ></ path > < path d = "M3 7.5h4" ></ path > < path d = "M3 12h18" ></ path > < path d = "M3 16.5h4" ></ path > < path d = "M17 3v18" ></ path > < path d = "M17 7.5h4" ></ path > < path d = "M17 16.5h4" ></ path > < / > } } pub const LUCIDE_FILM : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;