use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12.338 21.994A10 10 0 1 1 21.925 13.227" ></ path > < path d = "M12 6v6l2 1" ></ path > < path d = "m14 18 4 4 4-4" ></ path > < path d = "M18 14v8" ></ path > < / > } } pub const LUCIDE_CLOCK_ARROW_DOWN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;