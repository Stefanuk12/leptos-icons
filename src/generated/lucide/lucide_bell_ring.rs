use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" ></ path > < path d = "M10.3 21a1.94 1.94 0 0 0 3.4 0" ></ path > < path d = "M4 2C2.8 3.7 2 5.7 2 8" ></ path > < path d = "M22 8c0-2.3-.8-4.3-2-6" ></ path > < / > } } pub const LUCIDE_BELL_RING : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;