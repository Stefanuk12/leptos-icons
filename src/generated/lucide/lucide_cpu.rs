use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" rx = "2" x = "4" width = "16" height = "16" ></ rect > < rect width = "6" y = "9" x = "9" rx = "1" height = "6" ></ rect > < path d = "M15 2v2" ></ path > < path d = "M15 20v2" ></ path > < path d = "M2 15h2" ></ path > < path d = "M2 9h2" ></ path > < path d = "M20 15h2" ></ path > < path d = "M20 9h2" ></ path > < path d = "M9 2v2" ></ path > < path d = "M9 20v2" ></ path > < / > } } pub const LUCIDE_CPU : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;