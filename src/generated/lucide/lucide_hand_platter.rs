use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3V2" ></ path > < path d = "M5 10a7.1 7.1 0 0 1 14 0" ></ path > < path d = "M4 10h16" ></ path > < path d = "M2 14h12a2 2 0 1 1 0 4h-2" ></ path > < path d = "m15.4 17.4 3.2-2.8a2 2 0 0 1 2.8 2.9l-3.6 3.3c-.7.8-1.7 1.2-2.8 1.2h-4c-1.1 0-2.1-.4-2.8-1.2L5 18" ></ path > < path d = "M5 14v7H2" ></ path > < / > } } pub const LucideHandPlatter : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;