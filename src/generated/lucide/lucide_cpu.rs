use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" width = "16" height = "16" x = "4" rx = "2" ></ rect > < rect height = "6" width = "6" x = "9" y = "9" ></ rect > < path d = "M15 2v2" ></ path > < path d = "M15 20v2" ></ path > < path d = "M2 15h2" ></ path > < path d = "M2 9h2" ></ path > < path d = "M20 15h2" ></ path > < path d = "M20 9h2" ></ path > < path d = "M9 2v2" ></ path > < path d = "M9 20v2" ></ path > < / > } } pub const LucideCpu : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;