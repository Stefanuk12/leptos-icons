use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 2v6h6" ></ path > < path d = "M21 12A9 9 0 0 0 6 5.3L3 8" ></ path > < path d = "M21 22v-6h-6" ></ path > < path d = "M3 12a9 9 0 0 0 15 6.7l3-2.7" ></ path > < circle r = "1" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_REFRESH_CCW_DOT : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;