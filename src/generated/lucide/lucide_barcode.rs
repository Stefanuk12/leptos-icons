use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 5v14" ></ path > < path d = "M8 5v14" ></ path > < path d = "M12 5v14" ></ path > < path d = "M17 5v14" ></ path > < path d = "M21 5v14" ></ path > < / > } } pub const LUCIDE_BARCODE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;