use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12 7 2" ></ path > < path d = "m7 12 5-10" ></ path > < path d = "m12 12 5-10" ></ path > < path d = "m17 12 5-10" ></ path > < path d = "M4.5 7h15" ></ path > < path d = "M12 16v6" ></ path > < / > } } pub const LucideAntenna : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;