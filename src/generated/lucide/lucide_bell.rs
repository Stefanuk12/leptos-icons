use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" ></ path > < path d = "M10.3 21a1.94 1.94 0 0 0 3.4 0" ></ path > < / > } } pub const LUCIDE_BELL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;