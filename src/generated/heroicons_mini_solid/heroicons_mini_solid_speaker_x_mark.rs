use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.047 3.062a.75.75 0 0 1 .453.688v12.5a.75.75 0 0 1-1.264.546L5.203 13H2.667a.75.75 0 0 1-.7-.48A6.985 6.985 0 0 1 1.5 10c0-.887.165-1.737.468-2.52a.75.75 0 0 1 .7-.48h2.535l4.033-3.796a.75.75 0 0 1 .811-.142ZM13.78 7.22a.75.75 0 1 0-1.06 1.06L14.44 10l-1.72 1.72a.75.75 0 0 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L16.56 10l1.72-1.72a.75.75 0 0 0-1.06-1.06L15.5 8.94l-1.72-1.72Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_SPEAKER_X_MARK : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 20 20")] } ;