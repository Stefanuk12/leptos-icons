use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 16V4a2 2 0 0 1 2-2h11" ></ path > < path d = "M5 14H4a2 2 0 1 0 0 4h1" ></ path > < path d = "M22 18H11a2 2 0 1 0 0 4h11V6H11a2 2 0 0 0-2 2v12" ></ path > < / > } } pub const LUCIDE_BOOK_COPY : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;