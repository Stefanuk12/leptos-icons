use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "17" r = "1" ></ circle > < path d = "M21 7v6h-6" ></ path > < path d = "M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7" ></ path > < / > } } pub const LUCIDE_REDO_DOT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;