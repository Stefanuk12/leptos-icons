use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" ></ path > < polyline points = "7 10 12 15 17 10" ></ polyline > < line x2 = "12" x1 = "12" y1 = "15" y2 = "3" ></ line > < / > } } pub const LUCIDE_DOWNLOAD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;