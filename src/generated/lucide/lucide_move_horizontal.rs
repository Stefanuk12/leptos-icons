use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "18 8 22 12 18 16" ></ polyline > < polyline points = "6 8 2 12 6 16" ></ polyline > < line x2 = "22" y2 = "12" y1 = "12" x1 = "2" ></ line > < / > } } pub const LUCIDE_MOVE_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;