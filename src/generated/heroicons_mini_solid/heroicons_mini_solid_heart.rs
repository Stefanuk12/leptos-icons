use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9.653 16.915-.005-.003-.019-.01a20.759 20.759 0 0 1-1.162-.682 22.045 22.045 0 0 1-2.582-1.9C4.045 12.733 2 10.352 2 7.5a4.5 4.5 0 0 1 8-2.828A4.5 4.5 0 0 1 18 7.5c0 2.852-2.044 5.233-3.885 6.82a22.049 22.049 0 0 1-3.744 2.582l-.019.01-.005.003h-.002a.739.739 0 0 1-.69.001l-.002-.001Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_HEART : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("data-slot" , "icon")] } ;