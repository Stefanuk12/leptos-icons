use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 7v6h-6" ></ path > < path d = "M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7" ></ path > < / > } } pub const LUCIDE_REDO : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;