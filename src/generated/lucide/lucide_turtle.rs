use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 10 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a8 8 0 1 0-16 0v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3l2-4h4Z" ></ path > < path d = "M4.82 7.9 8 10" ></ path > < path d = "M15.18 7.9 12 10" ></ path > < path d = "M16.93 10H20a2 2 0 0 1 0 4H2" ></ path > < / > } } pub const LucideTurtle : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;