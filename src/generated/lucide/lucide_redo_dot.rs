use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "17" cx = "12" r = "1" ></ circle > < path d = "M21 7v6h-6" ></ path > < path d = "M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7" ></ path > < / > } } pub const LUCIDE_REDO_DOT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;