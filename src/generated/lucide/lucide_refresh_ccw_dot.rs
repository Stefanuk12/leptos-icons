use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 2v6h6" ></ path > < path d = "M21 12A9 9 0 0 0 6 5.3L3 8" ></ path > < path d = "M21 22v-6h-6" ></ path > < path d = "M3 12a9 9 0 0 0 15 6.7l3-2.7" ></ path > < circle r = "1" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_REFRESH_CCW_DOT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;