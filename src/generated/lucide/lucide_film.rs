use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" width = "18" height = "18" y = "3" ></ rect > < path d = "M7 3v18" ></ path > < path d = "M3 7.5h4" ></ path > < path d = "M3 12h18" ></ path > < path d = "M3 16.5h4" ></ path > < path d = "M17 3v18" ></ path > < path d = "M17 7.5h4" ></ path > < path d = "M17 16.5h4" ></ path > < / > } } pub const LUCIDE_FILM : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;