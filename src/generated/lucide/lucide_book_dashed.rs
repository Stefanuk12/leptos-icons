use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 22h-2" ></ path > < path d = "M20 15v2h-2" ></ path > < path d = "M4 19.5V15" ></ path > < path d = "M20 8v3" ></ path > < path d = "M18 2h2v2" ></ path > < path d = "M4 11V9" ></ path > < path d = "M12 2h2" ></ path > < path d = "M12 22h2" ></ path > < path d = "M12 17h2" ></ path > < path d = "M8 22H6.5a2.5 2.5 0 0 1 0-5H8" ></ path > < path d = "M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8" ></ path > < / > } } pub const LUCIDE_BOOK_DASHED : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;