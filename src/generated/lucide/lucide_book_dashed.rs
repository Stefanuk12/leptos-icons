use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17h2" ></ path > < path d = "M12 22h2" ></ path > < path d = "M12 2h2" ></ path > < path d = "M18 22h1a1 1 0 0 0 1-1" ></ path > < path d = "M18 2h1a1 1 0 0 1 1 1v1" ></ path > < path d = "M20 15v2h-2" ></ path > < path d = "M20 8v3" ></ path > < path d = "M4 11V9" ></ path > < path d = "M4 19.5V15" ></ path > < path d = "M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8" ></ path > < path d = "M8 22H6.5a1 1 0 0 1 0-5H8" ></ path > < / > } } pub const LUCIDE_BOOK_DASHED : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;