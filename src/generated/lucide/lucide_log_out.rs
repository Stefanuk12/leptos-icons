use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" ></ path > < polyline points = "16 17 21 12 16 7" ></ polyline > < line x2 = "9" y1 = "12" y2 = "12" x1 = "21" ></ line > < / > } } pub const LUCIDE_LOG_OUT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;