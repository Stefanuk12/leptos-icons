use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 22a1 1 0 0 1-1-1v-4a1 1 0 0 1 .445-.832l3-2a1 1 0 0 1 1.11 0l3 2A1 1 0 0 1 22 17v4a1 1 0 0 1-1 1z" ></ path > < path d = "M18 10a8 8 0 0 0-16 0c0 4.993 5.539 10.193 7.399 11.799a1 1 0 0 0 .601.2" ></ path > < path d = "M18 22v-3" ></ path > < circle cy = "10" cx = "10" r = "3" ></ circle > < / > } } pub const LUCIDE_MAP_PIN_HOUSE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none")] } ;