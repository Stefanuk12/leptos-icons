use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" ></ path > < polyline points = "16 17 21 12 16 7" ></ polyline > < line y2 = "12" x1 = "21" y1 = "12" x2 = "9" ></ line > < / > } } pub const LUCIDE_LOG_OUT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24")] } ;