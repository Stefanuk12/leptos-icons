use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" ></ path > < path d = "M14 8H8" ></ path > < path d = "M16 12H8" ></ path > < path d = "M13 16H8" ></ path > < / > } } pub const LUCIDE_RECEIPT_TEXT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;