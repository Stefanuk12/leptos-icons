use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" ></ path > < path d = "m12 10 3-3" ></ path > < path d = "m9 7 3 3v7.5" ></ path > < path d = "M9 11h6" ></ path > < path d = "M9 15h6" ></ path > < / > } } pub const LUCIDE_RECEIPT_JAPANESE_YEN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor")] } ;