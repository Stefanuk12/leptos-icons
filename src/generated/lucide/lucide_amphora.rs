use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 2v5.632c0 .424-.272.795-.653.982A6 6 0 0 0 6 14c.006 4 3 7 5 8" ></ path > < path d = "M10 5H8a2 2 0 0 0 0 4h.68" ></ path > < path d = "M14 2v5.632c0 .424.272.795.652.982A6 6 0 0 1 18 14c0 4-3 7-5 8" ></ path > < path d = "M14 5h2a2 2 0 0 1 0 4h-.68" ></ path > < path d = "M18 22H6" ></ path > < path d = "M9 2h6" ></ path > < / > } } pub const LUCIDE_AMPHORA : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;