use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0" ></ path > < path d = "M12 7v6" ></ path > < path d = "M9 10h6" ></ path > < / > } } pub const LUCIDE_MAP_PIN_PLUS_INSIDE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;