use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9.31 9.31 5 21l7-4 7 4-1.17-3.17" ></ path > < path d = "M14.53 8.88 12 2l-1.17 3.17" ></ path > < line y2 = "22" x2 = "22" y1 = "2" x1 = "2" ></ line > < / > } } pub const LUCIDE_NAVIGATION_2_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;