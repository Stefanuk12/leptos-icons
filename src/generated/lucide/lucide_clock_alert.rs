use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6v6l4 2" ></ path > < path d = "M16 21.16a10 10 0 1 1 5-13.516" ></ path > < path d = "M20 11.5v6" ></ path > < path d = "M20 21.5h.01" ></ path > < / > } } pub const LUCIDE_CLOCK_ALERT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;