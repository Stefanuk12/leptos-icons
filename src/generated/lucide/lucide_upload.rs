use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" ></ path > < polyline points = "17 8 12 3 7 8" ></ polyline > < line x2 = "12" y1 = "3" y2 = "15" x1 = "12" ></ line > < / > } } pub const LUCIDE_UPLOAD : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;