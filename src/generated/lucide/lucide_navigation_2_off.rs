use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9.31 9.31 5 21l7-4 7 4-1.17-3.17" ></ path > < path d = "M14.53 8.88 12 2l-1.17 3.17" ></ path > < line y1 = "2" y2 = "22" x1 = "2" x2 = "22" ></ line > < / > } } pub const LUCIDE_NAVIGATION_2_OFF : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;