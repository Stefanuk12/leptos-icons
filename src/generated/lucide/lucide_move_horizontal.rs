use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "18 8 22 12 18 16" ></ polyline > < polyline points = "6 8 2 12 6 16" ></ polyline > < line x2 = "22" y1 = "12" y2 = "12" x1 = "2" ></ line > < / > } } pub const LUCIDE_MOVE_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;