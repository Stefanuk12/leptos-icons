use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" ></ path > < polyline points = "7 10 12 15 17 10" ></ polyline > < line y1 = "15" x2 = "12" y2 = "3" x1 = "12" ></ line > < / > } } pub const LUCIDE_DOWNLOAD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;