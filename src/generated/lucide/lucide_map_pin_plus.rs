use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19.914 11.105A7.298 7.298 0 0 0 20 10a8 8 0 0 0-16 0c0 4.993 5.539 10.193 7.399 11.799a1 1 0 0 0 1.202 0 32 32 0 0 0 .824-.738" ></ path > < circle r = "3" cx = "12" cy = "10" ></ circle > < path d = "M16 18h6" ></ path > < path d = "M19 15v6" ></ path > < / > } } pub const LUCIDE_MAP_PIN_PLUS : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;