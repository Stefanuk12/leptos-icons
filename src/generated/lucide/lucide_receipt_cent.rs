use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" ></ path > < path d = "M12 6.5v11" ></ path > < path d = "M15 9.4a4 4 0 1 0 0 5.2" ></ path > < / > } } pub const LUCIDE_RECEIPT_CENT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;