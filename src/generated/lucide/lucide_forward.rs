use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "15 17 20 12 15 7" ></ polyline > < path d = "M4 18v-2a4 4 0 0 1 4-4h12" ></ path > < / > } } pub const LUCIDE_FORWARD : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;