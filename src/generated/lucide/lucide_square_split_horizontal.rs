use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 19H5c-1 0-2-1-2-2V7c0-1 1-2 2-2h3" ></ path > < path d = "M16 5h3c1 0 2 1 2 2v10c0 1-1 2-2 2h-3" ></ path > < line x1 = "12" y1 = "4" x2 = "12" y2 = "20" ></ line > < / > } } pub const LUCIDE_SQUARE_SPLIT_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;