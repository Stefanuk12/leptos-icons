use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" ></ path > < path d = "M8 7h8" ></ path > < path d = "M12 17.5 8 15h1a4 4 0 0 0 0-8" ></ path > < path d = "M8 11h8" ></ path > < / > } } pub const LUCIDE_RECEIPT_INDIAN_RUPEE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;