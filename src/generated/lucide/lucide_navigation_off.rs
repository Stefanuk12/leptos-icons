use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8.43 8.43 3 11l8 2 2 8 2.57-5.43" ></ path > < path d = "M17.39 11.73 22 2l-9.73 4.61" ></ path > < line x1 = "2" y2 = "22" x2 = "22" y1 = "2" ></ line > < / > } } pub const LUCIDE_NAVIGATION_OFF : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;