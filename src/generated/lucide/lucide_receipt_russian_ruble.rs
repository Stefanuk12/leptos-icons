use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" ></ path > < path d = "M8 15h5" ></ path > < path d = "M8 11h5a2 2 0 1 0 0-4h-3v10" ></ path > < / > } } pub const LUCIDE_RECEIPT_RUSSIAN_RUBLE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;