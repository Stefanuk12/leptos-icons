use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" width = "16" height = "16" rx = "2" y = "4" ></ rect > < rect height = "6" y = "9" rx = "1" width = "6" x = "9" ></ rect > < path d = "M15 2v2" ></ path > < path d = "M15 20v2" ></ path > < path d = "M2 15h2" ></ path > < path d = "M2 9h2" ></ path > < path d = "M20 15h2" ></ path > < path d = "M20 9h2" ></ path > < path d = "M9 2v2" ></ path > < path d = "M9 20v2" ></ path > < / > } } pub const LUCIDE_CPU : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;