use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18.977 14C19.6 12.701 20 11.343 20 10a8 8 0 0 0-16 0c0 4.993 5.539 10.193 7.399 11.799a1 1 0 0 0 1.202 0 32 32 0 0 0 .824-.738" ></ path > < circle r = "3" cy = "10" cx = "12" ></ circle > < path d = "M16 18h6" ></ path > < / > } } pub const LUCIDE_MAP_PIN_MINUS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24")] } ;